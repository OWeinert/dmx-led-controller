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
    public async Task<StatusResponse> PushAnimationsAsync(IEnumerable<Animation> animations, CancellationToken cancellationToken = default)
    {
        var dtos = animations.Select(anim => anim.ToDto());
        using var call = _dlclDrawClient.PushAnimations(cancellationToken: cancellationToken);
        foreach (var dto in dtos)
        {
            await call.RequestStream.WriteAsync(dto, cancellationToken);
        }
        return await call;
    }

    /// <summary>
    /// Sends a list of <paramref name="pixels"/> to be drawn on a layer with the specified <paramref name="layerId"/>
    /// to the DLCL-Server
    /// </summary>
    /// <param name="pixels"></param>
    /// <param name="layerId"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    /// <exception cref="NotImplementedException"></exception>
    [Obsolete("Not yet implemented!")]
    public async Task<StatusResponse> DrawOnLayerAsync(IEnumerable<Pixel> pixels, uint layerId, CancellationToken cancellationToken = default)
    {
        var dtos = pixels.Select(p => new LayerPixelDTO
        {
            Layer = layerId,
            Pixel = p.ToDto()
        });
        using var call = _dlclDrawClient.DrawOnLayer(cancellationToken: cancellationToken);
        foreach (var dto in dtos)
        {
            await call.RequestStream.WriteAsync(dto, cancellationToken: cancellationToken);
        }
        return await call;
    }
    
    /// <summary>
    /// Sends a full layer represented as <paramref name="singleFrame"/> to be drawn
    /// on a layer with the specified <paramref name="layerId"/> to the DLCL-Server
    /// </summary>
    /// <param name="singleFrame"></param>
    /// <param name="layerId"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    /// <exception cref="NotImplementedException"></exception>
    [Obsolete("Not yet implemented!")]
    public async Task<StatusResponse> DrawFullLayerAsync(SingleFrame singleFrame, uint layerId, CancellationToken cancellationToken = default)
    {
        var request = new DrawLayerRequest
        {
            Frame = singleFrame.ToDto(),
            Layer = layerId
        };
        var response = await _dlclDrawClient.DrawFullLayerAsync(request, cancellationToken: cancellationToken);
        return response;
    }
    
    /// <summary>
    /// Sends a list of <paramref name="pixels"/> to be drawn directly to the global framebuffer
    /// to the DLCL-Server
    /// </summary>
    /// <param name="pixels"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    /// <exception cref="NotImplementedException"></exception>
    [Obsolete("Not yet implemented!")]
    public async Task<StatusResponse> DrawDirectAsync(IEnumerable<Pixel> pixels, CancellationToken cancellationToken = default)
    {
        var dtos = pixels.Select(p => p.ToDto());
        var call = _dlclDrawClient.DrawDirect();
        foreach (var dto in dtos)
        {
            await call.RequestStream.WriteAsync(dto, cancellationToken);
        }
        return await call;
    }

    /// <summary>
    /// Retrieves a list of all Animations currently in the Animation Queue
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
