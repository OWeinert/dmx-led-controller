using DlclNet.Extensions;
using DlclRpc;
using DlclNet.Models;
using Grpc.Core;
using Grpc.Net.Client;

namespace DlclNet;
public class DlclClient
{
    private readonly DlclDraw.DlclDrawClient _dlclDrawClient;
    
    public DlclClient(string address)
    {
        using var channel = GrpcChannel.ForAddress(address);
        _dlclDrawClient = new DlclDraw.DlclDrawClient(channel);
    }

    /// <summary>
    /// Retrieves an Array of Ids of the DLCL-Server's animated layers
    /// </summary>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    public async Task<uint[]> GetAnimatedLayersAsync(CancellationToken cancellationToken = default)
    {
        var response = await _dlclDrawClient.GetAnimatedLayersAsync(new EmptyRequest(), cancellationToken: cancellationToken);
        var ids = response.Layers.ToArray();
        return ids;
    }

    /// <summary>
    /// Sends the list of <paramref name ="animations"/> as a stream to the DLCL-Server
    /// </summary>
    /// <param name="animations"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    public async Task<StatusResponse> PushAnimationsAsync(List<Animation> animations, CancellationToken cancellationToken = default)
    {
        var dtos = animations.ConvertAll(anim => anim.ToDto());
        using var call = _dlclDrawClient.PushAnimations(cancellationToken: cancellationToken);
        foreach (var dto in dtos)
        {
            await call.RequestStream.WriteAsync(dto, cancellationToken);
        }
        var response = await call;
        return response;
    }

    [Obsolete("Not yet implemented!")]
    public async Task<StatusResponse> DrawOnLayer(IEnumerable<Pixel> pixels, uint layerId, CancellationToken cancellationToken = default)
    {
        throw new NotImplementedException();
    }
    
    [Obsolete("Not yet implemented!")]
    public async Task<StatusResponse> DrawFullLayer(SingleFrame singleFrame, uint layerId, CancellationToken cancellationToken = default)
    {
        throw new NotImplementedException();
    }
    
    [Obsolete("Not yet implemented!")]
    public async Task<StatusResponse> DrawDirect(IEnumerable<Pixel> pixels, CancellationToken cancellationToken = default)
    {
        throw new NotImplementedException();
    }

    /// <summary>
    /// Retrieves a List of all Animations currently in the Animation Queue
    /// </summary>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    public async Task<List<Animation>> GetAnimationQueueAsync(CancellationToken cancellationToken = default)
    {
        var dtos = new List<AnimationDTO>();
        using var call = _dlclDrawClient.GetAnimationQueue(new EmptyRequest(), cancellationToken: cancellationToken);
        await foreach (var dto in call.ResponseStream.ReadAllAsync(cancellationToken))
        {
            dtos.Add(dto);
        }
        var animations = dtos.ConvertAll(dto => dto.ToAnimation());
        return animations;
    }
}
