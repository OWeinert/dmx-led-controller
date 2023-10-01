using DlclNet.Models;
using DlclRpc;

namespace DlclNet.Hosting;

/// <summary>
/// DLCL Client Service for use in Hosting (e.g. with ASP.Net or other hosted systems)
/// </summary>
public interface IDlclClientService
{
    /// <summary>
    ///<inheritdoc cref="DlclClient.GetAnimatedLayersAsync"/>
    /// </summary>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<uint[]> GetAnimatedLayersAsync(CancellationToken cancellationToken = default);

    /// <summary>
    /// <inheritdoc cref="DlclClient.PushAnimationsAsync"/>
    /// </summary>
    /// <param name="animations"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<StatusResponse> PushAnimationsAsync(List<Animation> animations, CancellationToken cancellationToken = default);
}